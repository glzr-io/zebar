import { Breadcrumbs, BreadcrumbsProps } from '@glzr/components';
import { IconHome } from '@tabler/icons-solidjs';
import { createMemo } from 'solid-js';

export function AppBreadcrumbs(props: BreadcrumbsProps) {
  const entries = createMemo(() => {
    return [
      {
        href: '/',
        content: () => <IconHome class="h-4 w-4 my-0.5" />,
      },
      ...props.entries,
    ];
  });

  return <Breadcrumbs class="mb-5" entries={entries()} />;
}
