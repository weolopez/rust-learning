class ItemsManager extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.items = [];
        this.apiUrl = window.APP_CONFIG.apiUrl;
    }

    connectedCallback() {
        this.render();
        this.fetchItems();
        this.setupEventListeners();
    }

    async fetchItems() {
        try {
            const response = await fetch(this.apiUrl);
            if (!response.ok) throw new Error('Failed to fetch items');
            this.items = await response.json();
            this.renderItems();
        } catch (error) {
            console.error('Error fetching items:', error);
            this.showError('Failed to load items. Is the API server running?');
        }
    }

    async addItem(name) {
        try {
            const response = await fetch(this.apiUrl, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name })
            });
            if (!response.ok) throw new Error('Failed to add item');
            const newItem = await response.json();
            this.items.push(newItem);
            this.renderItems();
        } catch (error) {
            console.error('Error adding item:', error);
            this.showError('Failed to add item');
        }
    }

    async toggleItem(id, completed) {
        try {
            const response = await fetch(`${this.apiUrl}/${id}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ completed })
            });
            if (!response.ok) throw new Error('Failed to update item');
            const updatedItem = await response.json();
            const index = this.items.findIndex(item => item.id === id);
            if (index !== -1) {
                this.items[index] = updatedItem;
                this.renderItems();
            }
        } catch (error) {
            console.error('Error updating item:', error);
            this.showError('Failed to update item');
        }
    }

    async deleteItem(id) {
        try {
            const response = await fetch(`${this.apiUrl}/${id}`, {
                method: 'DELETE'
            });
            if (!response.ok) throw new Error('Failed to delete item');
            this.items = this.items.filter(item => item.id !== id);
            this.renderItems();
        } catch (error) {
            console.error('Error deleting item:', error);
            this.showError('Failed to delete item');
        }
    }

    setupEventListeners() {
        const form = this.shadowRoot.querySelector('form');
        form.addEventListener('submit', (e) => {
            e.preventDefault();
            const input = this.shadowRoot.querySelector('#new-item-name');
            const name = input.value.trim();
            if (name) {
                this.addItem(name);
                input.value = '';
            }
        });
    }

    showError(message) {
        const errorDiv = this.shadowRoot.querySelector('.error');
        errorDiv.textContent = message;
        errorDiv.style.display = 'block';
        setTimeout(() => {
            errorDiv.style.display = 'none';
        }, 3000);
    }

    renderItems() {
        const list = this.shadowRoot.querySelector('.items-list');
        list.innerHTML = this.items.map(item => `
            <div class="item ${item.completed ? 'completed' : ''}">
                <input type="checkbox" 
                    ${item.completed ? 'checked' : ''} 
                    onchange="this.getRootNode().host.toggleItem(${item.id}, this.checked)">
                <span class="name">${item.name}</span>
                <button class="delete-btn" onclick="this.getRootNode().host.deleteItem(${item.id})">Delete</button>
            </div>
        `).join('');
    }

    render() {
        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                    font-family: Arial, sans-serif;
                    max-width: 500px;
                    margin: 20px auto;
                    padding: 20px;
                    border: 1px solid #ddd;
                    border-radius: 8px;
                    background-color: #fff;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                }
                h2 {
                    margin-top: 0;
                    color: #333;
                }
                form {
                    display: flex;
                    gap: 10px;
                    margin-bottom: 20px;
                }
                input[type="text"] {
                    flex: 1;
                    padding: 8px;
                    border: 1px solid #ddd;
                    border-radius: 4px;
                }
                button {
                    padding: 8px 16px;
                    background-color: #2196F3;
                    color: white;
                    border: none;
                    border-radius: 4px;
                    cursor: pointer;
                }
                button:hover {
                    background-color: #1976D2;
                }
                .items-list {
                    display: flex;
                    flex-direction: column;
                    gap: 10px;
                }
                .item {
                    display: flex;
                    align-items: center;
                    gap: 10px;
                    padding: 10px;
                    background-color: #f9f9f9;
                    border-radius: 4px;
                }
                .item.completed .name {
                    text-decoration: line-through;
                    color: #888;
                }
                .name {
                    flex: 1;
                }
                .delete-btn {
                    background-color: #f44336;
                    padding: 4px 8px;
                    font-size: 0.8em;
                }
                .delete-btn:hover {
                    background-color: #d32f2f;
                }
                .error {
                    color: #f44336;
                    margin-bottom: 10px;
                    display: none;
                    padding: 10px;
                    background-color: #ffebee;
                    border-radius: 4px;
                }
            </style>
            <h2>Items Manager</h2>
            <div class="error"></div>
            <form>
                <input type="text" id="new-item-name" placeholder="Add new item..." required>
                <button type="submit">Add</button>
            </form>
            <div class="items-list"></div>
        `;
    }
}

customElements.define('items-manager', ItemsManager);