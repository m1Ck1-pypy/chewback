import { Router, Route, Navigate } from "@solidjs/router";
import { AuthProvider } from "./context/AuthContext";
import { useAuth } from "./hooks/useAuth";
import { lazy, Show, type JSX } from "solid-js";

const Home = lazy(() => import("./pages/HomePage"));
const Auth = lazy(() => import("./pages/AuthPage"));

// Компонент защиты роута
const ProtectedRoute = (props: { children: JSX.Element }) => {
  const auth = useAuth();

  return (
    <Show
      when={!auth.isLoading()}
      fallback={<div class="app-loading">Loading...</div>}
    >
      <Show when={auth.user()} fallback={<Navigate href="/login" />}>
        {props.children}
      </Show>
    </Show>
  );
};

function App() {
  return (
    <AuthProvider>
      <Router>
        {/* Публичные маршруты */}
        <Route path="/login" component={Auth} />

        {/* Защищенные маршруты с Layout */}
        <Route
          path="/"
          component={() => <ProtectedRoute children={<Home />} />}
        />
        {/*<Route
          path="/profile"
          component={() => <ProtectedRoute children={<Profile />} />}
        />*/}
      </Router>
    </AuthProvider>
  );
}

export default App;
