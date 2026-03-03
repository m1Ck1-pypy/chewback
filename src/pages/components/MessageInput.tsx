interface MessageInputProps {
  value: string;
  onInput: (value: string) => void;
  onSubmit: (e: Event) => void;
}

export default function MessageInput(props: MessageInputProps) {
  return (
    <form
      class="p-4 border-t border-surface-800 flex gap-2"
      onSubmit={props.onSubmit}
    >
      <input
        type="text"
        class="input flex-1"
        placeholder="Type a message..."
        value={props.value}
        onInput={(e) => props.onInput(e.currentTarget.value)}
      />
      <button type="submit" class="btn btn-primary">
        <svg
          class="w-5 h-5"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"
          />
        </svg>
        Send
      </button>
    </form>
  );
}
