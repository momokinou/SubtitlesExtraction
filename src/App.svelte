<script lang="ts">
  import {
    Header,
    HeaderUtilities,
    HeaderAction,
    HeaderGlobalAction,
    HeaderPanelLinks,
    HeaderPanelDivider,
    HeaderPanelLink,
    SideNav,
    SideNavItems,
    SideNavMenu,
    SideNavMenuItem,
    SideNavLink,
    SkipToContent,
    Content,
    Grid,
    Row,
    Column,
    Theme,
  } from "carbon-components-svelte";
  import SelectFolder from "./lib/components/SelectFolder.svelte";
  import { Router, Link, Route } from "svelte-navigator";
  import Settings from "./routes/Settings.svelte";
  import Index from "./routes/index.svelte";
  let isSideNavOpen = false;
  let isOpen2 = false;
</script>

<main class="container">
  <Theme persist persistKey="carbon_theme">
    <Router primary={false}>
      <Header
        company="KHub"
        platformName="Subtitles Extractor"
        bind:isSideNavOpen
      />

      <SideNav bind:isOpen={isSideNavOpen}>
        <SideNavItems>
          <SideNavLink href="login" text="Link 1" />
          <SideNavLink href="index" text="Link 2" />
          <SideNavLink href="settings" text="Settings" />
        </SideNavItems>
      </SideNav>

      <Content>
        <Grid>
          <Row>
            <Column>
              <Route path="login">
                <SelectFolder />
              </Route>
              <Route path="index">
                <Index />
              </Route>
              <Route path="settings">
                <Settings />
              </Route>
            </Column>
          </Row>
        </Grid>
      </Content>
    </Router>
  </Theme>
</main>

<style>
</style>
