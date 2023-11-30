<script>
	import Atom from "../../svg/Atom.svelte";

    let isModalOpen = false;

    let selectedChain = "neutron-1"

    let chains = {
        'neutron-1': {
            display: "Neutron",
            tokens: [
                'ATOM',
                'stATOM'
            ]
        },

        'osmosis-1': {
            display: "Osmosis",
            tokens: [
                'ATOM',
                'stATOM'
            ]
        }
    };

    export let network = "Osmosis"
    export let token = "ATOM"

    function toggleModal() {
        isModalOpen = !isModalOpen;
    }

</script>


<button class="flex-col flex-grow h-16 btn btn-ghost bg-base-300" on:click={toggleModal}>
    <div class="flex items-center space-x-3">
        <Atom size={24} />
        <div class="space-y-2 text-left">
            <p>{network}</p>
            <p class="font-extralight">{token}</p>
        </div>
    </div>
</button>

<dialog class="modal" class:modal-open={isModalOpen}>
    <div class="max-w-xl modal-box">
        <div class="tabs">
            {#each Object.entries(chains) as [chainID, chainInfo]}
                <a class="tab tab-bordered" class:tab-active={chainID == selectedChain} on:click={() => selectedChain = chainID} >{chainInfo.display}</a> 
            {/each} 
        </div>
        <div class="modal-action">
            <form method="dialog">
            <!-- if there is a button in form, it will close the modal -->
            <button class="btn" on:click={toggleModal}>Close</button>
            </form>
        </div>
    </div>
</dialog>