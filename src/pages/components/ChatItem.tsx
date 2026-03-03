import type { Chat } from "./types";

interface ChatItemProps {
  chat: Chat;
  isSelected: boolean;
  onClick: () => void;
}

export default function ChatItem(props: ChatItemProps) {
  return (
    <button
      class={`w-full p-2 flex items-center gap-3 hover:bg-surface-900 transition-colors border-b border-surface-800/50 ${
        props.isSelected ? "bg-surface-900" : ""
      }`}
      onClick={props.onClick}
    >
      <div class="avatar avatar-md shrink-0">{props.chat.avatar}</div>
      <div class="flex-1 text-left min-w-0">
        <div class="flex items-center justify-between mb-1">
          <span class="text-sm font-medium text-surface-200 truncate">
            {props.chat.name}
          </span>
          <span class="text-xs text-surface-500">{props.chat.timestamp}</span>
        </div>
        <div class="flex justify-between items-center w-full gap-2">
          <p class="text-xs text-surface-400 truncate">
            {props.chat.lastMessage}
          </p>
          {props.chat.unread > 0 && (
            <span class="badge badge-success shrink-0">
              {props.chat.unread}
            </span>
          )}
        </div>
      </div>
    </button>
  );
}
