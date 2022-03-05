<template>
  <header>
    <nav
        class="navbar has-shadow"
        role="navigation"
        aria-label="main navigation"
    >
      <HeaderBarLinks/>
    </nav>
  </header>
</template>

<script>
import HeaderBarLinks from '@/components/header-bar-links';

export default {
  name: 'HeaderBar',
  components: {HeaderBarLinks},
};
</script>