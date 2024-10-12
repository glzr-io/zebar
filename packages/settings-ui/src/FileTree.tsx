import {
  IconChevronDown,
  IconChevronRight,
  IconFile,
  IconFolder,
} from '@glzr/components';
import { createSignal } from 'solid-js';

type FileType = 'file' | 'folder';

export interface FileItem {
  name: string;
  type: FileType;
  size?: string;
  modified?: string;
  children?: FileItem[];
}

export interface FileTreeProps {
  files: FileItem[];
  onSelect: (file: FileItem) => void;
}

export function FileTree({ files, onSelect }: FileTreeProps) {
  const [expanded, setExpanded] = createSignal<Record<string, boolean>>(
    {},
  );

  const toggleExpand = (name: string) => {
    setExpanded(prev => ({ ...prev, [name]: !prev[name] }));
  };

  const renderFileItem = (file: FileItem, level: number) => {
    const isExpanded = expanded()[file.name];
    const hasChildren = file.children && file.children.length > 0;

    return (
      <div class="select-none">
        <div
          class="flex items-center gap-1 px-2 py-1 hover:bg-gray-100 cursor-pointer"
          style={{ 'padding-left': `${level * 16}px` }}
          onClick={() => {
            if (hasChildren) {
              toggleExpand(file.name);
            }
            onSelect(file);
          }}
        >
          {hasChildren && (
            <button
              onClick={() => toggleExpand(file.name)}
              class="focus:outline-none"
            >
              {isExpanded ? (
                <IconChevronDown class="w-4 h-4" />
              ) : (
                <IconChevronRight class="w-4 h-4" />
              )}
            </button>
          )}
          <FileIcon type={file.type} />
          <span>{file.name}</span>
        </div>

        {hasChildren && isExpanded && (
          <div>
            {file.children!.map(child => renderFileItem(child, level + 1))}
          </div>
        )}
      </div>
    );
  };

  return <div>{files.map(file => renderFileItem(file, 0))}</div>;
}

const FileIcon = ({ type }: { type: FileType }) => {
  switch (type) {
    case 'folder':
      return <IconFolder class="w-4 h-4" />;
    default:
      return <IconFile class="w-4 h-4" />;
  }
};
