import orbLogo from './assets/OrbLogo.svg';
import { useWebSocket } from './hooks/useWebsocket';
import './style/App.scss';

function App() {

	const { socket, isConnected } = useWebSocket('ws://localhost:3030');

	return (
		<>
			<div className="flex gap-4 items-center">
				<div className="">
					<img
						src={orbLogo}
						className="logo"
						alt="This is a weird looking logo"
					/>
				</div>
				<h1 className="font-bold font-mono no-select text-slate-500 tracking-wide">
					KTRL
				</h1>
			</div>
			<div>
				{
					isConnected ? <h1>Connected To Websocket</h1> : <h1>Not Connected</h1>
				}
			</div>
		</>
	);
}

export default App;
