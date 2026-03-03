import { createSignal } from "solid-js";
import { api } from "../../api";
import { useAuth } from "../../hooks/useAuth";

interface RegisterFormProps {
  onToggleMode: () => void;
  onSuccess?: () => void;
  error?: string | null;
}

export default function RegisterForm(props: RegisterFormProps) {
  const auth = useAuth();

  const [login, setLogin] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [confirmPassword, setConfirmPassword] = createSignal("");
  const [localError, setLocalError] = createSignal<string | null>(null);
  const [isLoading, setIsLoading] = createSignal(false);

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    setLocalError(null);

    // Client-side validation
    if (login().length < 3) {
      setLocalError("Login must be at least 3 characters");
      return;
    }
    if (password().length < 6) {
      setLocalError("Password must be at least 6 characters");
      return;
    }
    if (password() !== confirmPassword()) {
      setLocalError("Passwords do not match");
      return;
    }

    setIsLoading(true);

    try {
      await api.auth.register({
        login: login(),
        password: password(),
      });
      await auth.checkAuth();
      props.onSuccess?.();
    } catch (err) {
      setLocalError(err instanceof Error ? err.message : "Registration failed");
    } finally {
      setIsLoading(false);
    }
  };

  const error = () => props.error || localError();

  return (
    <>
      <h2 class="heading-lg text-center mb-2">Create Account</h2>
      <p class="body-md text-center mb-6 text-surface-400">
        Sign up to get started with Chewback
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
            disabled={isLoading()}
          />
        </div>

        <div>
          <label for="confirm-password" class="label">
            Confirm Password
          </label>
          <input
            id="confirm-password"
            type="password"
            class="input"
            value={confirmPassword()}
            onInput={(e) => setConfirmPassword(e.currentTarget.value)}
            placeholder="Confirm your password"
            disabled={isLoading()}
          />
        </div>

        {error() && (
          <div class="badge badge-error w-full justify-center">{error()}</div>
        )}

        <button
          type="submit"
          class="btn btn-primary w-full"
          disabled={isLoading()}
        >
          {isLoading() ? (
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
            "Sign Up"
          )}
        </button>
      </form>

      <div class="divider" />

      <p class="body-sm text-center">
        Already have an account?{" "}
        <button
          type="button"
          class="link bg-transparent p-0"
          onClick={props.onToggleMode}
          disabled={isLoading()}
        >
          Sign In
        </button>
      </p>
    </>
  );
}
