const socket = new WebSocket('ws://localhost:8080/ws/');

socket.onmessage = (event) => {
  console.log("Message from server:", event.data);
};

export const sendMessage = (message) => {
  socket.send(JSON.stringify({ type: "message", content: message }));
};

export default socket;
