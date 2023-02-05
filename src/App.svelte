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
  import ViewFiles from "./routes/viewFiles.svelte";
  import About from "./routes/about.svelte";
  import ExtractSub from "./routes/ExtractSub.svelte";
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
          <SideNavLink href="choseDir" text="Chose directory" />
          <SideNavLink href="viewFiles" text="View Files" />
          <SideNavLink href="process" text="Extract sub" />
          <SideNavLink href="settings" text="Settings" />
          <SideNavLink href="about" text="About" />
        </SideNavItems>
      </SideNav>

      <Content>
        <Grid>
          <Row>
            <Column>
              <Route path="choseDir">
                <SelectFolder />
              </Route>
              <Route path="viewFiles">
                <ViewFiles />
              </Route>
              <Route path="process">
                <ExtractSub />
              </Route>
              <Route path="settings">
                <Settings />
              </Route>
              <Route path="about">
                <About />
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
