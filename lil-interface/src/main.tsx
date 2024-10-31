import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './style/index.scss';
import App from './App.tsx';
import '@mantine/core/styles.css';
import { MantineProvider } from '@mantine/core';

createRoot(document.getElementById('root')!).render(
	<MantineProvider defaultColorScheme="dark">
		<StrictMode>
			<App />
		</StrictMode>
	</MantineProvider>
);
