import Header from './components/header';
import { useGCSConnection } from './data/ws.singleton';
import Live from './routes/live';
import '@/style/App.scss';
import '@/style/battery.scss';

useGCSConnection();

function App() {
	return (
		<div className="app">
			<Header />
			<Live />
		</div>
	);
}

export default App;
