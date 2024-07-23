<script lang="ts">
    import { page } from "$app/stores";


</script>

<header class="flex justify-center items-center">
    <nav>
         
        <ul>
            <li aria-current={$page.url.pathname === "/" ? "page" : undefined}>
                <a href="/">Home</a>
            </li>
            <li
                aria-current={$page.url.pathname === "/timer"
                    ? "page"
                    : undefined}
            >
                <a href="/todo">Todo</a>
            </li>
        </ul>
         
    </nav>
</header>
