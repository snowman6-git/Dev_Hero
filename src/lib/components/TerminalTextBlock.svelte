<script lang="ts">
    let { 
        text, 
        typing = false, 
    } = $props<{ 
        text: string,
        typing?: boolean,
    }>();

</script>

<div class="text-container">
    {text}
</div>

<style>
    .text-container {
        color: white;
		margin-bottom: 0.25rem;
    }
</style>