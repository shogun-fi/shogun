<script>
    import "../app.css"

    import { page } from '$app/stores';
</script>

<div class="absolute navbar bg-base-100">
    <div class="flex-1">
      <a href="/" class="text-xl tracking-widest font-extralight btn btn-ghost text-accent">軍 SHOGUN</a>
      <ul class="menu menu-horizontal">
        <li><a href="/swap">Swap</a></li>
        <li class="disabled"><p>Liquidity <span class="text-xs tracking-wide font-extralight badge badge-accent">COMING SOON</span></p></li>
      </ul>
    </div>
    <div class="flex-none">
      <button class="text-sm font-bold tracking-wider text-accent btn btn-ghost">
        Connect
      </button>
    </div>
</div>
<slot />