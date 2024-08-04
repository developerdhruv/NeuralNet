document.addEventListener("DOMContentLoaded", () => {
    const form = document.getElementById("chat-form");
    const input = document.getElementById("user-input");
    const chatBox = document.getElementById("chat-box");

    form.addEventListener("submit", async (e) => {
        e.preventDefault();

        const userMessage = input.value;
        appendMessage("user-message", userMessage);
        input.value = "";

        try {
            const response = await axios.get('http://localhost:8000/chat', {
                params: {
                    user_input: userMessage,
                    url: 'https://medium.com/llamaindex-blog/building-better-tools-for-llm-agents-f8c5a6714f11'
                }
            });

            appendMessage("bot-message", response.data.response);
        } catch (error) {
            console.error('Error:', error);
            appendMessage("bot-message", "Sorry, there was an error processing your request.");
        }
    });

    function appendMessage(className, message) {
        const messageDiv = document.createElement("div");
        messageDiv.className = `chat-message ${className}`;
        messageDiv.textContent = message;
        chatBox.appendChild(messageDiv);
        chatBox.scrollTop = chatBox.scrollHeight;
    }
});
