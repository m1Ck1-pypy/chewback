import { createSignal, createEffect, onCleanup } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { useAuth } from "../hooks/useAuth";
import type { Chat, Message } from "./components/types";
import ChatList from "./components/ChatList";
import ChatDialog from "./components/ChatDialog";
import { api } from "../api";

const mockChats: Chat[] = [
  {
    id: 1,
    name: "Alice Johnson",
    lastMessage: "Hey, how are you doing?",
    timestamp: "10:30 AM",
    unread: 2,
    avatar: "AJ",
  },
  {
    id: 2,
    name: "Bob Smith",
    lastMessage: "Let's meet tomorrow",
    timestamp: "09:15 AM",
    unread: 0,
    avatar: "BS",
  },
  {
    id: 3,
    name: "Carol White",
    lastMessage: "Thanks for the help!",
    timestamp: "Yesterday",
    unread: 1,
    avatar: "CW",
  },
];

const mockMessages: Record<number, Message[]> = {
  1: [
    { id: 1, text: "Hi there!", sender: "other", timestamp: "10:25 AM" },
    { id: 2, text: "How's it going?", sender: "other", timestamp: "10:26 AM" },
    {
      id: 3,
      text: "Pretty good, thanks!",
      sender: "me",
      timestamp: "10:28 AM",
    },
    {
      id: 4,
      text: "Hey, how are you doing?",
      sender: "other",
      timestamp: "10:30 AM",
    },
    { id: 5, text: "What about you?", sender: "me", timestamp: "10:31 AM" },
  ],
  2: [
    {
      id: 1,
      text: "We need to discuss the project",
      sender: "other",
      timestamp: "09:00 AM",
    },
    {
      id: 2,
      text: "Sure, when are you free?",
      sender: "me",
      timestamp: "09:10 AM",
    },
    {
      id: 3,
      text: "Let's meet tomorrow",
      sender: "other",
      timestamp: "09:15 AM",
    },
  ],
};

export default function HomePage() {
  const navigate = useNavigate();
  const { isAuthenticated, logout } = useAuth();

  // Redirect if not authenticated
  if (!isAuthenticated()) {
    navigate("/auth");
    return null;
  }

  const [selectedChat, setSelectedChat] = createSignal<Chat | null>(
    mockChats[0],
  );
  const [messageInput, setMessageInput] = createSignal("");
  const [sidebarCollapsed, setSidebarCollapsed] = createSignal(false);

  // Auto-collapse sidebar on small screens (< 500px)
  createEffect(() => {
    const checkScreenSize = () => {
      if (window.innerWidth < 500) {
        setSidebarCollapsed(true);
      }
    };

    checkScreenSize();
    window.addEventListener("resize", checkScreenSize);
    onCleanup(() => window.removeEventListener("resize", checkScreenSize));
  });

  const currentMessages = () => {
    const chat = selectedChat();
    return chat ? mockMessages[chat.id] || [] : [];
  };

  const handleSendMessage = async (e: Event) => {
    e.preventDefault();
    const text = messageInput();
    if (!text.trim()) return;
    setMessageInput("");

    try {
      const response = await api.auth.getMe();
      console.log(response);
    } catch (error) {
      console.error(error);
    }
  };

  const toggleSidebar = () => {
    setSidebarCollapsed(!sidebarCollapsed());
  };

  const handleLogout = async () => {
    try {
      await logout();
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <main class="min-h-screen bg-surface-950 flex flex-col">
      {/* Top Bar */}
      <header class="border-b border-surface-800 bg-surface-glass sticky top-0 z-50">
        <div class="container-base py-3 flex items-center justify-between">
          <h1 class="heading-sm">
            <span class="text-gradient-primary">Chewback</span>
          </h1>
          <button onClick={handleLogout} class="btn btn-outline btn-sm">
            Logout
          </button>
        </div>
      </header>

      {/* Chat Area */}
      <div class="flex-1 flex overflow-hidden">
        <ChatList
          chats={mockChats}
          selectedChat={selectedChat()}
          onSelectChat={(chat) => setSelectedChat(chat)}
          collapsed={sidebarCollapsed()}
        />
        <ChatDialog
          selectedChat={selectedChat()}
          messages={currentMessages()}
          messageInput={messageInput()}
          onMessageInput={setMessageInput}
          onSendMessage={handleSendMessage}
          onToggleSidebar={toggleSidebar}
        />
      </div>
    </main>
  );
}
