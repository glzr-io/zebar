import { Breadcrumbs, BreadcrumbsProps } from '@glzr/components';
import { createMemo } from 'solid-js';

export function AppBreadcrumbs(props: BreadcrumbsProps) {
  const entries = createMemo(() => {
    return [
      {
        href: '/',
        content: 'Home',
      },
      ...props.entries,
    ];
  });

  return <Breadcrumbs entries={entries()} />;
}
