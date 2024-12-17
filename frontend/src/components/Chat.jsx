import React, { useState, useEffect } from "react";
import socket, { sendMessage } from "../api/websocket";

const Chat = () => {
  const [messages, setMessages] = useState([]);
  const [input, setInput] = useState("");

  useEffect(() => {
    socket.onmessage = (event) => {
      const data = JSON.parse(event.data);
      setMessages((prev) => [...prev, data]);
    };
  }, []);

  const handleSend = () => {
    sendMessage(input);
    setInput("");
  };

  return (
    <div>
      <div className="chat-box">
        {messages.map((msg, index) => (
          <div key={index}>{msg.content}</div>
        ))}
      </div>
      <input
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        placeholder="Type a message"
      />
      <button onClick={handleSend}>Send</button>
    </div>
  );
};

export default Chat;
