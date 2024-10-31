import orbLogo from './assets/OrbLogo.svg';
import './style/App.scss';

function App() {
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
		</>
	);
}

export default App;
