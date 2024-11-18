import Header from './components/header';
import { GCS_Connection } from './data/ws.singleton';
import Live from './routes/live';
import '@/style/App.scss';
import '@/style/battery.scss';

GCS_Connection();

function App() {
	return (
		<div className="app">
			<Header />
			<Live />
		</div>
	);
}

export default App;
