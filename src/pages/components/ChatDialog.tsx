import type { Chat, Message } from "./types";
import MessageBubble from "./MessageBubble";
import MessageInput from "./MessageInput";

interface ChatDialogProps {
  selectedChat: Chat | null;
  messages: Message[];
  messageInput: string;
  onMessageInput: (value: string) => void;
  onSendMessage: (e: Event) => void;
  onToggleSidebar: () => void;
}

export default function ChatDialog(props: ChatDialogProps) {
  return (
    <section class="flex-1 flex flex-col">
      {props.selectedChat ? (
        <>
          {/* Chat Header */}
          <div class="p-4 border-b border-surface-800 flex items-center gap-3">
            <button
              class="btn btn-ghost btn-sm"
              onClick={props.onToggleSidebar}
            >
              <svg
                class="w-6 h-6"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M4 6h16M4 12h16M4 18h16"
                />
              </svg>
            </button>
            <div class="avatar avatar-md">{props.selectedChat.avatar}</div>
            <div>
              <h3 class="heading-sm">{props.selectedChat.name}</h3>
              <p class="text-xs text-surface-500">Online</p>
            </div>
          </div>

          {/* Messages */}
          <div class="flex-1 overflow-y-auto p-4 space-y-4">
            {props.messages.map((msg) => (
              <MessageBubble key={msg.id} message={msg} />
            ))}
          </div>

          {/* Message Input */}
          <MessageInput
            value={props.messageInput}
            onInput={props.onMessageInput}
            onSubmit={props.onSendMessage}
          />
        </>
      ) : (
        <div class="flex-1 flex items-center justify-center">
          <p class="text-surface-500">Select a chat to start messaging</p>
        </div>
      )}
    </section>
  );
}
