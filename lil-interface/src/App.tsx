import orbLogo from './assets/OrbLogo.svg';
import { useWebSocket } from './hooks/useWebsocket';
import { useLogStore } from './state/logstore';
import './style/App.scss';

function App() {

	const { isConnected } = useWebSocket('ws://localhost:3030');


    const log_message = useLogStore((state) => state.log_messages);

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
			<div className='flex flex-row gap-10 items-center w-full'>
			<div>
				{
					isConnected ? <h1>Connected To Websocket</h1> : <h1>Not Connected</h1>
				}
			</div>
			<div>
				{log_message}
			</div>

			</div>
		</>
	);
}

export default App;
