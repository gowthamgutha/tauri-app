<script>
    export let name = "";
    export let age = 1;
    export let country = "";
</script>

<div>
    <span class="header">{name}</span>
    <span class="body">
        Your age is {age} and country is {country}
    </span>
</div>

<style>
    div {
        border: 1px solid #ddd;
        border-top-left-radius: 0.3em;
        border-top-right-radius: 0.3em;
        display: inline-grid;
        align-self: center;
    }
    div > span.header {
        width: 100%;
        max-width: 100%;
        background: #efefef;
        border-bottom: 1px solid #ddd;
        font-weight: bolder;
    }
    div > span.body {
        height: 80%;
    }
</style>
