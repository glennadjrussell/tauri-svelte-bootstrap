<svelte:head>
  <script src="/js/color-modes.js"></script>
  <script src="/js/dashboard.js"></script>
  <script src="https://cdn.jsdelivr.net/npm/chart.js@4.3.2/dist/chart.umd.js" integrity="sha384-eI7PSr3L1XLISH8JdDII5YN/njoSsxfbrkCTnJrzXt+ENP5MOVBxD+l6sEG4zoLp" crossorigin="anonymous"></script>
</svelte:head>

<script>
  import { Container, Row, Col } from '@sveltestrap/sveltestrap';
  import { Nav, NavItem, Navbar, NavbarBrand, NavLink } from '@sveltestrap/sveltestrap';
  import { Table } from '@sveltestrap/sveltestrap';
</script>

<Navbar color="light">
  <NavbarBrand href="/">Metasast</NavbarBrand>
</Navbar>

<Container fluid>
  <Row>
    <Col md="3">
    <Nav vertical>
      <NavItem>
        <NavLink href="#">
	  <svg class="bi">
	    <use xlink:href="/assets/icons.svg#house-fill"></use>
	  </svg>
	  Dashboard
	</NavLink>
      </NavItem>
      <NavItem>
        <NavLink href="#">
	  <svg class="bi">
	    <use xlink:href="/assets/icons.svg#file-earmark"></use>
	  </svg>
	  Orders
	</NavLink>
      </NavItem>
      <NavItem>
        <NavLink href="#">
	  <svg class="bi">
	    <use xlink:href="/assets/icons.svg#cart"></use>
	  </svg>
	  Products
	</NavLink>
      </NavItem>
      <NavItem>
        <NavLink href="#">
	  <svg class="bi">
	    <use xlink:href="/assets/icons.svg#people"></use>
	  </svg>
	  Customers
	</NavLink>
      </NavItem>
      <NavItem>
        <NavLink href="#">
	  <svg class="bi">
	    <use xlink:href="/assets/icons.svg#graph-up"></use>
	  </svg>
	  Reports
	</NavLink>
      </NavItem>
      <NavItem>
        <NavLink href="#">
	  <svg class="bi">
	    <use xlink:href="/assets/icons.svg#puzzle"></use>
	  </svg>
	  Integrations
	</NavLink>
      </NavItem>
    </Nav>
    <h6><span>Saved reports</span></h6>
    <Nav vertical>
      <NavItem>
        <NavLink href="#">Current month</NavLink>
      </NavItem>
    </Nav>
    <hr class="my-3"/>
    <Nav vertical>
      <NavItem>
        <NavLink href="#">
	  <svg class="bi">
	    <use xlink:href="/assets/icons.svg#gear-wide-connected"></use>
	  </svg>
	  Settings
	</NavLink>
      </NavItem>
      <NavItem>
        <NavLink href="#">
	  <svg class="bi">
	    <use xlink:href="/assets/icons.svg#door-closed"></use>
	  </svg>
	  Sign out
	</NavLink>
      </NavItem>
    </Nav>
    </Col>
    <Col md="9">
      <main>
        <h1 class="h2">Dashboard</h1>

	<canvas class="my-4 w-100" id="myChart" width="900" height="380"></canvas>

	<h2>Section title</h2>
	<Table striped size="sm">
          <thead>
            <tr>
              <th scope="col">#</th>
              <th scope="col">Header</th>
              <th scope="col">Header</th>
              <th scope="col">Header</th>
              <th scope="col">Header</th>
            </tr>
          </thead>

	</Table>
      </main>
    </Col>
  </Row>
</Container>

<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@docsearch/css@3">
<link href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.10.5/font/bootstrap-icons.min.css" rel="stylesheet">
<link rel="stylesheet" href="/styles/dashboard.css"/>
<link rel="stylesheet" href="/styles/mainpage.css"/>
<link rel="stylesheet" href="/styles/bootstrap.min.css"/>
<link rel="stylesheet" href="/styles/bootstrap.min.css.map"/>

