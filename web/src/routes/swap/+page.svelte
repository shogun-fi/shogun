<script>
	import PairSelector from "$components/swap/PairSelector.svelte";
</script>

<div class="flex flex-col justify-center h-screen">
    <div class="container mx-auto">
        <div class="flex items-center justify-center">
        <div>
            <div class="shadow-xl bg-base-200 card w-96">
            <div class="card-body">
                <div class="flex flex-col w-full space-y-5">
                    <PairSelector />
                <input type="text" placeholder="0" class="flex-grow input input-lg" />
                <div class="justify-center card-actions">
                    <button class="font-light tracking-wider text-md btn btn-primary btn-block">Swap</button> 
                </div>
                <p class="font-mono text-xs text-center text-red-500">• NOT CONNECTED</p>
                </div>
            </div>
            </div>
        </div>
        </div>
    </div>
</div>