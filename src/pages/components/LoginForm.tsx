import { createSignal } from "solid-js";
import { api } from "../../api";
import { useAuth } from "../../hooks/useAuth";

interface LoginFormProps {
  onToggleMode: () => void;
  onSuccess?: () => void;
  loading?: boolean;
  error?: string | null;
}

export default function LoginForm(props: LoginFormProps) {
  const auth = useAuth();
  const [login, setLogin] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [localError, setLocalError] = createSignal<string | null>(null);
  const [isLoading, setIsLoading] = createSignal(false);

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    setLocalError(null);

    const user = login();
    const pass = password();

    if (!user || !pass) {
      setLocalError("All fields are required");
      return;
    }

    try {
      await api.auth.login({
        login: login(),
        password: password(),
      });

      await auth.checkAuth(); // Это обновит состояние пользователя
      props.onSuccess?.();
    } catch (err) {
      setLocalError(err instanceof Error ? err.message : "Login failed");
    } finally {
      setIsLoading(false);
    }
  };

  const error = () => props.error || localError();
  const loading = () => props.loading || false;

  return (
    <>
      <h2 class="heading-lg text-center mb-2">Welcome Back</h2>
      <p class="body-md text-center mb-6 text-surface-400">
        Sign in to continue to your account
      </p>

      <form class="flex flex-col gap-4" onSubmit={handleSubmit}>
        <div>
          <label for="username" class="label">
            Username
          </label>
          <input
            id="username"
            type="text"
            class="input"
            value={login()}
            onInput={(e) => setLogin(e.currentTarget.value)}
            placeholder="Enter your username"
            disabled={isLoading()}
          />
        </div>

        <div>
          <label for="password" class="label">
            Password
          </label>
          <input
            id="password"
            type="password"
            class="input"
            value={password()}
            onInput={(e) => setPassword(e.currentTarget.value)}
            placeholder="Enter your password"
            disabled={loading()}
          />
        </div>

        {error() && (
          <div class="badge badge-error w-full justify-center">{error()}</div>
        )}

        <button
          type="submit"
          class="btn btn-primary w-full"
          disabled={loading()}
        >
          {loading() ? (
            <span class="flex items-center gap-2">
              <svg class="animate-spin w-5 h-5" fill="none" viewBox="0 0 24 24">
                <circle
                  class="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  stroke-width="4"
                />
                <path
                  class="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                />
              </svg>
              Processing...
            </span>
          ) : (
            "Sign In"
          )}
        </button>
      </form>

      <div class="divider" />

      <p class="body-sm text-center">
        Don't have an account?{" "}
        <button
          type="button"
          class="link bg-transparent p-0"
          onClick={props.onToggleMode}
          disabled={loading()}
        >
          Sign Up
        </button>
      </p>
    </>
  );
}
