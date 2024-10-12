import { FileItem, FileTree } from './FileTree';
import { WidgetSettings } from './WidgetSettings';

const initialFiles: FileItem[] = [
  {
    name: 'Documents',
    type: 'folder',
    children: [
      {
        name: 'Project Proposal.docx',
        type: 'file',
        size: '2.3 MB',
        modified: '2023-10-05',
      },
      {
        name: 'Budget.xlsx',
        type: 'file',
        size: '1.8 MB',
        modified: '2023-10-06',
      },
    ],
  },
  {
    name: 'Images',
    type: 'folder',
    children: [
      {
        name: 'Vacation.jpg',
        type: 'image',
        size: '3.2 MB',
        modified: '2023-09-28',
      },
      {
        name: 'Family.png',
        type: 'image',
        size: '2.9 MB',
        modified: '2023-09-29',
      },
    ],
  },
  {
    name: 'Music.mp3',
    type: 'audio',
    size: '5.4 MB',
    modified: '2023-10-01',
  },
  {
    name: 'Video.mp4',
    type: 'video',
    size: '15.7 MB',
    modified: '2023-10-02',
  },
];

export function App() {
  return (
    <div class="app">
      <FileTree files={initialFiles} onSelect={() => {}} />
      <WidgetSettings />
    </div>
  );
}
