import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App.tsx';
import './style/index.scss';
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';
import '@mantine/charts/styles.css';
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
