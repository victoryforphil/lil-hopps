import { ActionIcon, Badge } from '@mantine/core'
import { IconArrowLeft, IconDots, IconPlugConnected } from '@tabler/icons-react'

export default function SidebarHeader() {
  return (
    <div className='flex flex-1 justify-between items-center'>
        <div className='flex items-center'>
            <ActionIcon variant="subtle" aria-label="Back to overview" color='gray' className='mr-2'>
                <IconArrowLeft style={{ width: '70%', height: '70%' }} stroke={1.5} />
            </ActionIcon>
            <div className='flex gap-2 items-center'>
                <div>
                    Drone
                </div>
                <div className='text-xs font-mono opacity-50 pt-2'>
                    v.0.0.1
                </div>
            </div>
            <Badge color='green' size='xs' className='ml-4'>Live</Badge>
        </div>

        <div className='flex gap-2'>
            <ActionIcon variant="filled" aria-label="Options" color='red'>
                <IconPlugConnected style={{ width: '70%', height: '70%' }} stroke={1.5} />
            </ActionIcon>
            <ActionIcon variant="filled" aria-label="Options" color='gray'>
                <IconDots style={{ width: '70%', height: '70%' }} stroke={1.5} />
            </ActionIcon>
        </div>

    </div>
  )
}
