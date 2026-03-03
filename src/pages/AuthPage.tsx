import { createSignal } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { useAuth } from "../hooks/useAuth";
import LoginForm from "./components/LoginForm";
import RegisterForm from "./components/RegisterForm";

type AuthMode = "login" | "register";

export default function AuthPage() {
  const navigate = useNavigate();
  const auth = useAuth();

  const [mode, setMode] = createSignal<AuthMode>("login");

  const isLogin = () => mode() === "login";

  const handleSuccess = async () => {
    await auth.checkAuth(); // Обновляем состояние
    navigate("/"); // Редирект на главную
  };

  const toggleMode = () => {
    setMode(isLogin() ? "register" : "login");
  };

  return (
    <main class="min-h-screen bg-surface-950 flex items-center justify-center p-8">
      <div class="card w-full max-w-md">
        {isLogin() ? (
          <LoginForm onSuccess={handleSuccess} onToggleMode={toggleMode} />
        ) : (
          <RegisterForm onSuccess={handleSuccess} onToggleMode={toggleMode} />
        )}
      </div>
    </main>
  );
}
