export default function BigNumber(props: { val: number, name: string }) {
  return (
    <div className='flex flex-col items-center justify-between bg-[#1f1f1f] p-4 rounded-md w-28 flex-1'>
        <div className='text-green-500 text-xs font-mono w-fit ml-auto mb-2'>+16</div>
        <div className='font-mono text-3xl'>{props.val}</div>
        <div className="opacity-60 text-sm font-extralight">{props.name}</div>
    </div>
  )
}
