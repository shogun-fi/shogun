<script>
  import WalletConnect from '../svg/WalletConnect.svelte'

  import Atom from '../svg/Atom.svelte'
  import Usdt from '../svg/Usdt.svelte'
</script>

<div class="flex flex-col justify-between h-screen">
  <div class="container mx-auto">
    <div class="navbar bg-base-100">
      <div class="flex-1">
        <p class="text-xl tracking-widest normal-case font-extralight btn btn-ghost">軍 SHOGUN</p>
      </div>
      <div class="flex-none">
        <button class="btn btn-ghost bg-[#3B99FC] text-white tracking-wider font-bold">
          <WalletConnect />
          CONNECT
        </button>
      </div>
    </div>
    <div class="flex items-center justify-center">
      <div>
        <div class="p-10 text-center">
          <h1 class="text-6xl font-light text-white">軍</h1>
        </div>
        <div class="shadow-xl bg-base-200 card w-96">
          <div class="card-body">
            <div class="flex flex-col w-full space-y-5">
              <div class="flex flex-col w-full lg:flex-row">
                <details class="flex-grow dropdown bg-base-300 rounded-box ">
                  <summary class="h-full text-sm font-light btn btn-lg btn-ghost btn-block"><Atom size={20}/>ATOM</summary>
                  <ul class="p-2 shadow menu dropdown-content z-[1] bg-base-100 rounded-box w-52">
                  </ul>
                </details>
                <div class="text-xs divider lg:divider-horizontal">TO</div> 
                <details class="flex-grow dropdown bg-base-300 rounded-box ">
                  <summary class="h-full text-sm font-light btn btn-lg btn-ghost btn-block"><Usdt size={20}/>USDT</summary>
                  <ul class="p-2 shadow menu dropdown-content z-[1] bg-base-100 rounded-box w-52">
                  </ul>
                </details>
              </div>
              <input type="text" placeholder="0.0" class="flex-grow input input-lg" />
              <div class="justify-center card-actions">
                <button class="font-light tracking-wider text-md btn btn-primary btn-block">Swap</button> 
              </div>
              <p class="font-mono text-xs text-center text-green-300"><span class="animate-pulse">•</span> CONNECTED</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <footer class="p-10 rounded footer footer-center bg-base-200 text-base-content">
    <nav class="grid grid-flow-col gap-4">
      <a class="link link-hover">About</a> 
      <a class="link link-hover">Contact</a> 
      <a class="link link-hover">Press</a>
    </nav> 
    <a><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-current"><path d="M24 4.557c-.883.392-1.832.656-2.828.775 1.017-.609 1.798-1.574 2.165-2.724-.951.564-2.005.974-3.127 1.195-.897-.957-2.178-1.555-3.594-1.555-3.179 0-5.515 2.966-4.797 6.045-4.091-.205-7.719-2.165-10.148-5.144-1.29 2.213-.669 5.108 1.523 6.574-.806-.026-1.566-.247-2.229-.616-.054 2.281 1.581 4.415 3.949 4.89-.693.188-1.452.232-2.224.084.626 1.956 2.444 3.379 4.6 3.419-2.07 1.623-4.678 2.348-7.29 2.04 2.179 1.397 4.768 2.212 7.548 2.212 9.142 0 14.307-7.721 13.995-14.646.962-.695 1.797-1.562 2.457-2.549z"></path></svg></a> 
    <aside>
      <p>© 2023 Shogun Labs</p>
    </aside>
  </footer>
</div>