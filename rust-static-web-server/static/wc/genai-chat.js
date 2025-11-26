class GenAIChat extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this.shadowRoot.innerHTML = `
      <style>
        :host { --bg: #0f1724; --card: #0b1220; --accent: #7c5cff; --muted: #9aa4b2; --text: #e6eef7; display:block; font-family: Inter, ui-sans-serif, system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial; }
        .wrapper{background: linear-gradient(180deg, rgba(124,92,255,0.08), rgba(0,0,0,0)) , var(--bg); padding:18px; border-radius:12px; box-shadow: 0 6px 30px rgba(2,6,23,0.6); color:var(--text); max-width:720px; margin:12px auto}
        header{display:flex;align-items:center;gap:12px;margin-bottom:12px}
        .logo{width:44px;height:44px;border-radius:10px;background:linear-gradient(135deg,var(--accent),#00d4ff);display:flex;align-items:center;justify-content:center;font-weight:700}
        .title{font-size:16px;font-weight:600}
        .subtitle{font-size:12px;color:var(--muted)}
        .chat{background:linear-gradient(180deg, rgba(255,255,255,0.02), transparent); border-radius:10px; padding:12px; min-height:220px; max-height:420px; overflow:auto}
        .bubble{max-width:78%; padding:10px 14px;border-radius:12px;margin:8px 0; line-height:1.45}
        .user{margin-left:auto;background:linear-gradient(180deg, rgba(255,255,255,0.03), rgba(124,92,255,0.06)); border:1px solid rgba(124,92,255,0.12);}
        .ai{background:linear-gradient(180deg, rgba(0,0,0,0.04), rgba(255,255,255,0.02)); border:1px solid rgba(255,255,255,0.03);}
        form{display:flex; gap:8px; margin-top:12px}
        textarea{flex:1; resize:none; padding:12px 14px;border-radius:10px;border:1px solid rgba(255,255,255,0.04); background:transparent; color:var(--text); min-height:44px}
        button{background:var(--accent); color:white; border:none; padding:10px 14px; border-radius:10px; cursor:pointer; box-shadow: 0 6px 18px rgba(124,92,255,0.12)}
        .meta{display:flex; gap:8px; align-items:center; justify-content:space-between; margin-top:8px}
        .small{font-size:12px;color:var(--muted)}
        .spinner{width:18px;height:18px;border-radius:50%; border:2px solid rgba(255,255,255,0.08); border-top-color:var(--accent); animation:spin 0.8s linear infinite}
        @keyframes spin{to{transform:rotate(360deg)}}
      </style>
      <div class="wrapper">
        <header>
          <div class="logo">AI</div>
          <div>
            <div class="title">GenAI Chat</div>
            <div class="subtitle">Ask anything — uses /prompt API</div>
          </div>
        </header>
        <div class="chat" id="chat" role="log"></div>
        <form id="form">
          <textarea id="input" placeholder="Type your message..." aria-label="message"></textarea>
          <button id="send" type="submit">Send</button>
        </form>
        <div class="meta"><div class="small">Responses are streamed from the server</div><div id="status" class="small"></div></div>
      </div>
    `;

    this.chatEl = this.shadowRoot.getElementById('chat');
    this.inputEl = this.shadowRoot.getElementById('input');
    this.formEl = this.shadowRoot.getElementById('form');
    this.statusEl = this.shadowRoot.getElementById('status');

    this.formEl.addEventListener('submit', (e)=> this.onSubmit(e));
  }

  connectedCallback(){
    // allow pressing Enter to send
    this.inputEl.addEventListener('keydown', (e)=>{
      if(e.key === 'Enter' && !e.shiftKey){
        e.preventDefault();
        this.formEl.dispatchEvent(new Event('submit', {cancelable:true}));
      }
    });
  }

  appendBubble(text, who='ai'){
    const b = document.createElement('div');
    b.className = 'bubble ' + (who==='user' ? 'user' : 'ai');
    b.textContent = text;
    this.chatEl.appendChild(b);
    this.chatEl.scrollTop = this.chatEl.scrollHeight;
    return b;
  }

  setStatus(msg){
    this.statusEl.textContent = msg;
  }

  async onSubmit(e){
    e.preventDefault();
    const prompt = this.inputEl.value.trim();
    if(!prompt) return;
    // show user bubble
    this.appendBubble(prompt, 'user');
    this.inputEl.value = '';
    this.setStatus('Sending...');

    // create placeholder for streaming AI response
    const placeholder = this.appendBubble('...', 'ai');
    placeholder.textContent = '';
    this.setStatus('Waiting for response');

    try{
      const apiUrl = this.getAttribute('api-url') || '/prompt';
      const resp = await fetch(apiUrl, {
        method:'POST',
        headers: {'Content-Type':'application/json'},
        body: JSON.stringify({ prompt })
      });

      if(!resp.ok){
        const err = await resp.json().catch(()=>({error:'unknown'}));
        placeholder.textContent = 'Error: ' + (err.error || resp.statusText);
        this.setStatus('Error');
        return;
      }

      // try to parse as streaming JSON first
      // we'll handle plain JSON with `result` as fallback
      const reader = resp.body.getReader();
      const decoder = new TextDecoder();
      let done=false; let accumulated='';
      while(!done){
        const {value, done: d} = await reader.read();
        done = d;
        if(value){
          const chunk = decoder.decode(value, {stream:true});
          accumulated += chunk;
          // naive: try to parse any JSON fragments; many streaming endpoints send newline-delimited JSON
          // instead of well-formed JSON arrays — we look for JSON objects inside the accumulated text.
          let lastIdx = 0;
          for(let i=0;i<accumulated.length;i++){
            // look for end-of-object '}' followed by newline or end
            if(accumulated[i] === '}' ){
              const maybe = accumulated.slice(0, i+1);
              try{
                const parsed = JSON.parse(maybe);
                if(parsed.result) {
                  placeholder.textContent = parsed.result;
                } else if(parsed.candidate){
                  placeholder.textContent += parsed.candidate;
                }
                lastIdx = i+1;
              }catch(err){
                // not a full json object yet
              }
            }
          }
          if(lastIdx>0){
            accumulated = accumulated.slice(lastIdx);
          } else {
            // fallback: append raw chunk to placeholder for progressive text
            placeholder.textContent += chunk;
          }
          this.chatEl.scrollTop = this.chatEl.scrollHeight;
        }
      }

      // fallback: if server returned final JSON body
      if(!placeholder.textContent){
        const json = await resp.json().catch(()=>null);
        if(json && json.result) placeholder.textContent = json.result;
      }

      this.setStatus('Done');
    }catch(err){
      placeholder.textContent = 'Error: ' + (err.message || err);
      this.setStatus('Error');
    }
  }
}

customElements.define('genai-chat', GenAIChat);
