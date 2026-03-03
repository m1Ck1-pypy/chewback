import type { Message } from "./types";

interface MessageBubbleProps {
  message: Message;
}

export default function MessageBubble(props: MessageBubbleProps) {
  const { message } = props;
  
  return (
    <div
      class={`flex ${message.sender === "me" ? "justify-end" : "justify-start"}`}
    >
      <div
        class={`max-w-md px-4 py-2 rounded-lg ${
          message.sender === "me"
            ? "bg-primary-600 text-white"
            : "bg-surface-800 text-surface-200"
        }`}
      >
        <p>{message.text}</p>
        <p
          class={`text-xs mt-1 ${
            message.sender === "me" ? "text-primary-200" : "text-surface-500"
          }`}
        >
          {message.timestamp}
        </p>
      </div>
    </div>
  );
}
