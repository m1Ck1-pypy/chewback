/**
 * Tailwind CSS Component Templates для Chewback
 *
 * Этот файл содержит примеры использования всех доступных стилей.
 * Импортируйте классы из index.css в ваши компоненты.
 */

// ============================================
// ПРИМЕР: Карточка с кнопками
// ============================================
export function CardExample() {
  return (
    <div class="card-hover">
      <h3 class="heading-md mb-2">Заголовок карточки</h3>
      <p class="body-md mb-4">
        Описание карточки с каким-то контентом.
      </p>
      <div class="flex gap-2">
        <button class="btn-primary">Действие</button>
        <button class="btn-outline">Отмена</button>
      </div>
    </div>
  );
}

// ============================================
// ПРИМЕР: Форма с валидацией
// ============================================
export function FormExample() {
  return (
    <form class="card max-w-md">
      <div class="mb-4">
        <label class="label" for="email">Email</label>
        <input
          type="email"
          id="email"
          class="input"
          placeholder="name@example.com"
        />
      </div>

      <div class="mb-4">
        <label class="label" for="password">Пароль</label>
        <input
          type="password"
          id="password"
          class="input-error"
          placeholder="Минимум 8 символов"
        />
        <p class="body-sm text-red-400 mt-1">Пароль слишком короткий</p>
      </div>

      <div class="mb-4">
        <label class="label" for="confirm">Подтверждение</label>
        <input
          type="password"
          id="confirm"
          class="input-success"
          placeholder="Повторите пароль"
        />
        <p class="body-sm text-green-400 mt-1">Пароли совпадают</p>
      </div>

      <button type="submit" class="btn-primary w-full">
        Отправить
      </button>
    </form>
  );
}

// ============================================
// ПРИМЕР: Бейджи
// ============================================
export function BadgesExample() {
  return (
    <div class="flex gap-2 flex-wrap">
      <span class="badge-primary">Primary</span>
      <span class="badge-accent">Accent</span>
      <span class="badge-success">Success</span>
      <span class="badge-warning">Warning</span>
      <span class="badge-error">Error</span>
    </div>
  );
}

// ============================================
// ПРИМЕР: Кнопки
// ============================================
export function ButtonsExample() {
  return (
    <div class="flex gap-2 flex-wrap">
      <button class="btn-primary">Primary</button>
      <button class="btn-accent">Accent</button>
      <button class="btn-outline">Outline</button>
      <button class="btn-ghost">Ghost</button>

      <div class="divider" />

      <button class="btn-primary btn-sm">Small</button>
      <button class="btn-primary">Default</button>
      <button class="btn-primary btn-lg">Large</button>

      <div class="divider" />

      <button class="btn-primary" disabled>Disabled</button>
      <button class="btn-primary">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        С иконкой
      </button>
    </div>
  );
}

// ============================================
// ПРИМЕР: Типографика
// ============================================
export function TypographyExample() {
  return (
    <div class="space-y-4">
      <h1 class="heading-xl">Heading XL</h1>
      <h2 class="heading-lg">Heading LG</h2>
      <h3 class="heading-md">Heading MD</h3>
      <h4 class="heading-sm">Heading SM</h4>

      <p class="body-lg">Body Large — для основного текста</p>
      <p class="body-md">Body Medium — для описаний</p>
      <p class="body-sm">Body Small — для подписей</p>

      <a href="#" class="link">Ссылка с подчёркиванием</a>

      <p class="text-gradient-primary">Градиентный текст</p>
    </div>
  );
}

// ============================================
// ПРИМЕР: Аватары
// ============================================
export function AvatarExample() {
  return (
    <div class="flex gap-4 items-center">
      <div class="avatar-sm">A</div>
      <div class="avatar-md">B</div>
      <div class="avatar-lg">C</div>

      {/* С иконкой */}
      <div class="avatar-md">
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
        </svg>
      </div>
    </div>
  );
}

// ============================================
// ПРИМЕР: Tooltip
// ============================================
export function TooltipExample() {
  return (
    <div class="flex gap-4">
      <button class="btn-primary tooltip" data-tooltip="Подсказка сверху">
        Наведи на меня
      </button>
      <button class="btn-outline tooltip" data-tooltip="Ещё одна подсказка">
        И на меня
      </button>
    </div>
  );
}

// ============================================
// ПРИМЕР: Layout с контейнером
// ============================================
export function LayoutExample() {
  return (
    <div class="min-h-screen bg-surface-950">
      <header class="border-b border-surface-800">
        <div class="container-base py-4">
          <h1 class="heading-sm">Header</h1>
        </div>
      </header>

      <main class="container-base py-8">
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div class="card">Card 1</div>
          <div class="card">Card 2</div>
          <div class="card">Card 3</div>
        </div>
      </main>

      <footer class="border-t border-surface-800">
        <div class="container-base py-4">
          <p class="body-sm">Footer</p>
        </div>
      </footer>
    </div>
  );
}

// ============================================
// ПРИМЕР: Анимации
// ============================================
export function AnimationExample() {
  return (
    <div class="flex gap-4">
      <div class="card animate-once">
        Fade In (один раз)
      </div>
      <div class="card animate-pulse-slow">
        Pulse Slow
      </div>
      <div class="card animate-bounce-slow">
        Bounce Slow
      </div>
    </div>
  );
}

// ============================================
// ПРИМЕР: Glassmorphism
// ============================================
export function GlassExample() {
  return (
    <div class="relative">
      {/* Фоновый градиент для демонстрации стекла */}
      <div class="absolute inset-0 bg-gradient-radial from-primary-500/20 to-transparent" />

      <div class="relative bg-surface-glass card">
        <h3 class="heading-md mb-2">Glass Effect</h3>
        <p class="body-md">
          Полупрозрачный фон с размытием
        </p>
      </div>
    </div>
  );
}
