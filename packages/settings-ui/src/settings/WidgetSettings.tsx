import { FileItem, WidgetConfigTree } from './WidgetConfigTree';
import { WidgetSettingsForm } from './WidgetSettingsForm';

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
        type: 'file',
        size: '3.2 MB',
        modified: '2023-09-28',
      },
      {
        name: 'Family.png',
        type: 'file',
        size: '2.9 MB',
        modified: '2023-09-29',
      },
    ],
  },
  {
    name: 'Music.mp3',
    type: 'file',
    size: '5.4 MB',
    modified: '2023-10-01',
  },
  {
    name: 'Video.mp4',
    type: 'file',
    size: '15.7 MB',
    modified: '2023-10-02',
  },
];

export function WidgetSettings() {
  return (
    <div class="app">
      <WidgetConfigTree files={initialFiles} onSelect={() => {}} />
      <WidgetSettingsForm />
    </div>
  );
}
