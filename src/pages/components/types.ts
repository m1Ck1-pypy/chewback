export interface Chat {
  id: number;
  name: string;
  lastMessage: string;
  timestamp: string;
  unread: number;
  avatar: string;
}

export interface Message {
  id: number;
  text: string;
  sender: "me" | "other";
  timestamp: string;
}
