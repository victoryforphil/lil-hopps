import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './style/index.scss';
import App from './App.tsx';
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';
import { MantineProvider } from '@mantine/core';
import { Notifications } from '@mantine/notifications';

createRoot(document.getElementById('root')!).render(
	<MantineProvider defaultColorScheme="dark">
		<Notifications />
		<StrictMode>
			<App />
		</StrictMode>
	</MantineProvider>
);
