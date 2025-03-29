import remarkGfm from 'remark-gfm';
import { SolidMarkdown } from 'solid-markdown';

export type MarkdownProps = {
  children: string;
};

export function Markdown(props: MarkdownProps) {
  return (
    <SolidMarkdown
      children={props.children}
      remarkPlugins={[remarkGfm]}
      components={{
        code: props => (
          <code class="px-1.5 py-0.5 rounded bg-muted font-mono text-sm">
            {props.children}
          </code>
        ),
        h1: props => (
          <h1 class="text-2xl font-bold mt-6 mb-4">{props.children}</h1>
        ),
        h2: props => (
          <h2 class="text-xl font-bold mt-5 mb-3">{props.children}</h2>
        ),
        h3: props => (
          <h3 class="text-lg font-bold mt-4 mb-2">{props.children}</h3>
        ),
        h4: props => (
          <h4 class="text-base font-bold mt-3 mb-2">{props.children}</h4>
        ),
        h5: props => (
          <h5 class="text-sm font-bold mt-3 mb-1">{props.children}</h5>
        ),
        h6: props => (
          <h6 class="text-xs font-bold mt-3 mb-1">{props.children}</h6>
        ),
        li: props => <li class="ml-6 mb-1">{props.children}</li>,
        ol: props => (
          <ol class="list-decimal pl-6 my-4">{props.children}</ol>
        ),
        ul: props => <ul class="list-disc pl-6 my-4">{props.children}</ul>,
        td: props => <td class="border px-4 py-2">{props.children}</td>,
        th: props => (
          <th class="border px-4 py-2 font-bold bg-muted">
            {props.children}
          </th>
        ),
        tr: props => <tr class="border-b">{props.children}</tr>,
      }}
    />
  );
}
