<script lang='ts'>
    import silhouette from '$lib/images/slapshot_silhouette.png'
    
    let searchInput = "";
    let searchResults: [string, string[]][] = [];

    const data = {
        "1234": ["name 1", "name 2"],
        "5678": ["alias 1", "alias 2", "alias 3"],
        "359575": ["tom@to", "Matt"],
        "99039": ["Robo", "Aidan"],
        "729916": ["four_4", "Chris"],
        "4321": ["name 4", "name 3"],
        "2468": ["Row Bow", "Tom Mato", "fore_for"],
        "4756": ["Boat Goes Binted", "Bogos Binted"],
        "2025": ["<script>", "\<script\>console.log(test)\<\/script\>"]
    }

    const search = () => {
        searchResults = [];
        if (searchInput != "") {
            console.log(searchInput)
            searchResults = fuzzySearch(searchInput, data, 4);
        }
    }

    const fuzzySearch = (query: string, data: Record<string, string[]>, threshold: number): [string, string[]][] => {
        const results: [string, string[], number][] = [];

        for (const [id, aliases] of Object.entries(data)) {
            const idDist = levenshteinDistance(query.toLowerCase(), id.toLowerCase());
            const aliasDists = aliases.map(alias =>
                levenshteinDistance(query.toLowerCase(), alias.toLowerCase())
            );
            const minDist = Math.min(idDist, ...aliasDists);

            if (minDist <= threshold) {
                results.push([id, aliases, minDist]);
            }
        }

        return results
            .sort((a, b) => a[2] - b[2])
            .map(([id, alias, _]) => [id, alias]);
    }

    const levenshteinDistance = (a: string, b: string): number => {
        const aLen = a.length;
        const bLen = b.length;
        const dp: number[][] = [];

        for (let i = 0; i <= aLen; i++) {
            dp[i] = [i];
        }
        
        for (let j = 1; j <= bLen; j++) {
            dp[0][j] = j;
        }
        
        for (let j = 1; j <= bLen; j++) {
            for (let i = 1; i <= aLen; i++) {
                if (a[i - 1] === b[j - 1]) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = Math.min(
                        dp[i - 1][j] + 1, // Delete
                        dp[i][j - 1] + 1, // Insert
                        dp[i - 1][j - 1] + 1 // Sub
                    );
                }
            }
        }

        return dp[aLen][bLen];
    }
</script>

<div id="search-bar-container" class="w-full h-full relative">
    <div class="relative flex items-center rounded-md bg-white pl-3 outline-1 -outline-offset-1 outline-gray-300 has-[input:focus-within]:outline-2 has-[input:focus-within]:-outline-offset-2 has-[input:focus-within]:outline-blue-500">
        <div class="shrink-0 text-base text-gray-500 select-none sm:text-sm/6">
            <svg class="h-5 w-5 text-gray-500"  width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                <path stroke="none" d="M0 0h24v24H0z"/>
                <circle cx="10" cy="10" r="7" />
                <line x1="21" y1="21" x2="15" y2="15" />
            </svg>
        </div>
        <input bind:value={searchInput} on:input={() => search()} type="text" name="search" class="block min-w-0 grow py-1.5 pr-3 pl-1 text-base text-gray-900 placeholder:text-gray-400 focus:outline-none sm:text-sm/6" placeholder="SlapID or Username">
    </div>
    <!-- {#if searchResults.length > 0} -->
    {#if searchInput != ""}
    <div id="search-results" class="absolute max-h-[33vh] w-full bg-white rounded-sm text-wrap overflow-y-auto
        [&::-webkit-scrollbar]:w-2
        [&::-webkit-scrollbar-track]:rounded-full
        [&::-webkit-scrollbar-track]:bg-gray-100
        [&::-webkit-scrollbar-thumb]:rounded-full
        [&::-webkit-scrollbar-thumb]:bg-gray-300">
        {#if searchResults.length > 0}
            <div class="divide-y divide-gray-200">
                {#each searchResults as [id, names]}
                    <a href="/player/{id}">
                        <div class="flex items-center space-x-4 p-3 hover:bg-gray-200">
                            <div id="result-image" class="flex-shrink-0 w-[75px] h-[75px] overflow-hidden">
                                <img 
                                    class="w-full h-full object-cover" 
                                    src={silhouette} 
                                    alt="Silhouette" 
                                    width="75" 
                                    height="75"
                                >
                            </div>
                            <div class="flex-grow min-w-0">
                                <p class="text-sm font-medium text-gray-900 truncate">{names[0]}</p>
                                <p class="text-sm text-gray-500 truncate">Other Aliases: {names.slice(1).join(', ')}</p>
                                <p class="text-sm text-gray-500">Slap ID: {id}</p>
                            </div>
                        </div>
                    </a>
                {/each}
            </div>
        {:else}
            <p class="p-3 text-sm text-gray-500">No search results for "{searchInput}"</p>
        {/if}
        </div>
    {/if}
</div>