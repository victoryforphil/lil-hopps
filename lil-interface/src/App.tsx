import Header from './components/header';
import Live from './routes/live';
import '@/style/App.scss';
import '@/style/battery.scss';

function App() {
	return (
		<div className="app">
			<Header />
			<Live />
		</div>
	);
}

export default App;
