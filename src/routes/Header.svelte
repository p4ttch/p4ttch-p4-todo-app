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

<style>
    nav {
        display: flex;
    } 

    li {
        display: inline-block;
        height: 30px;
        line-height: 30px;
        padding: 10px 20px;
        text-transform: uppercase;
        position: relative;
    }
    a {
        text-decoration: none;
        border-radius: 4px;
        border:solid #888 1px;
        padding: 8px 12px;
    }
</style>
