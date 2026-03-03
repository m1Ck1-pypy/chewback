import type { Chat } from "./types";
import ChatItem from "./ChatItem";

interface ChatListProps {
  chats: Chat[];
  selectedChat: Chat | null;
  onSelectChat: (chat: Chat) => void;
  collapsed: boolean;
}

export default function ChatList(props: ChatListProps) {
  return (
    <aside
      class={`border-r border-surface-800 flex flex-col transition-all duration-300 ${
        props.collapsed ? "w-0 overflow-hidden" : "w-64"
      }`}
    >
      {/* Chat List Header */}
      <div class="p-4 border-b border-surface-800 shrink-0">
        <h2 class="heading-md">Chats</h2>
      </div>

      {/* Chat List */}
      <div class="flex-1 overflow-y-auto">
        {props.chats.map((chat) => (
          <ChatItem
            key={chat.id}
            chat={chat}
            isSelected={props.selectedChat?.id === chat.id}
            onClick={() => props.onSelectChat(chat)}
          />
        ))}
      </div>
    </aside>
  );
}
