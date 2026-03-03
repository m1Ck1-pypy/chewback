import type { JSX } from "solid-js";

interface LayoutProps {
  children?: JSX.Element;
}

export const Layout = (props: LayoutProps) => {
  return <div class="layout">{props.children}</div>;
};
