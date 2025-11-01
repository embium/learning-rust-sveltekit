<script lang="ts">
  import { auth } from "$lib/stores/auth";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import {
    ArrowRight,
    Check,
    Zap,
    Shield,
    Users,
    BarChart3,
    Github,
    Star,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";

  onMount(() => {
    // Redirect authenticated users to dashboard
    if ($auth.isAuthenticated) {
      goto(resolve("/dashboard"));
    }
  });

  const features = [
    {
      icon: Zap,
      title: "Fast & Reliable",
      description:
        "Built with modern technologies for optimal performance and reliability.",
    },
    {
      icon: Shield,
      title: "Secure by Default",
      description:
        "Enterprise-grade security with authentication and authorization built-in.",
    },
    {
      icon: Users,
      title: "Team Collaboration",
      description:
        "Work together seamlessly with your team members on projects.",
    },
    {
      icon: BarChart3,
      title: "Analytics & Insights",
      description:
        "Get detailed insights into your projects and team performance.",
    },
  ];

  const pricing = [
    {
      name: "Starter",
      price: "$0",
      period: "forever",
      description: "Perfect for individuals getting started",
      features: [
        "Up to 3 projects",
        "Basic analytics",
        "Email support",
        "1GB storage",
      ],
      popular: false,
    },
    {
      name: "Professional",
      price: "$19",
      period: "per month",
      description: "Ideal for growing teams",
      features: [
        "Unlimited projects",
        "Advanced analytics",
        "Priority support",
        "100GB storage",
        "Team collaboration",
        "Custom integrations",
      ],
      popular: true,
    },
    {
      name: "Enterprise",
      price: "$99",
      period: "per month",
      description: "For large organizations",
      features: [
        "Everything in Professional",
        "Dedicated support",
        "SSO integration",
        "Unlimited storage",
        "Advanced security",
        "Custom deployment",
      ],
      popular: false,
    },
  ];
</script>

<svelte:head>
  <title>SaaS Boilerplate - Modern Project Management</title>
  <meta
    name="description"
    content="A powerful SaaS platform built with SvelteKit and Rust for modern project management and team collaboration."
  />
</svelte:head>

<div class="min-h-screen">
  <!-- Hero Section -->
  <section
    class="relative overflow-hidden bg-linear-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800"
  >
    <div class="container mx-auto px-4 py-20 lg:py-32">
      <div class="text-center max-w-4xl mx-auto">
        <div class="mb-8 flex justify-center">
          <div
            class="inline-flex items-center rounded-full border px-3 py-1 text-sm font-medium bg-background/60 backdrop-blur"
          >
            <Star class="mr-1 h-3 w-3" />
            Trusted by 10,000+ teams worldwide
          </div>
        </div>

        <h1
          class="text-4xl font-bold tracking-tight sm:text-6xl lg:text-7xl mb-6"
        >
          The future of
          <span
            class="text-transparent bg-clip-text bg-linear-to-r from-blue-600 to-purple-600"
          >
            project management
          </span>
        </h1>

        <p
          class="text-xl text-muted-foreground mb-8 max-w-2xl mx-auto leading-relaxed"
        >
          Streamline your workflow, collaborate with your team, and deliver
          projects faster with our modern SaaS platform.
        </p>

        <div class="flex flex-col sm:flex-row gap-4 justify-center">
          <Button size="lg" href="/signup" class="text-lg px-8 py-6">
            Get Started Free
            <ArrowRight class="ml-2 h-5 w-5" />
          </Button>
          <Button
            variant="outline"
            size="lg"
            href="/login"
            class="text-lg px-8 py-6"
          >
            Sign In
          </Button>
        </div>

        <div class="mt-12 text-sm text-muted-foreground">
          No credit card required • 14-day free trial • Cancel anytime
        </div>
      </div>
    </div>

    <!-- Background decoration -->
    <div class="absolute inset-0 -z-10">
      <div
        class="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[800px] rounded-full bg-linear-to-br from-blue-400/20 to-purple-600/20 blur-3xl"
      ></div>
    </div>
  </section>

  <!-- Features Section -->
  <section class="py-20 lg:py-32">
    <div class="container mx-auto px-4">
      <div class="text-center mb-16">
        <h2 class="text-3xl font-bold tracking-tight sm:text-4xl mb-4">
          Everything you need to succeed
        </h2>
        <p class="text-xl text-muted-foreground max-w-2xl mx-auto">
          Powerful features designed to help you and your team work more
          efficiently.
        </p>
      </div>

      <div class="grid gap-8 md:grid-cols-2 lg:grid-cols-4">
        {#each features as feature (feature.title)}
          <Card.Root
            class="text-center border-0 shadow-lg hover:shadow-xl transition-shadow"
          >
            <Card.Content class="p-8">
              <div
                class="inline-flex items-center justify-center w-12 h-12 rounded-lg bg-primary/10 mb-4"
              >
                <svelte:component
                  this={feature.icon}
                  class="h-6 w-6 text-primary"
                />
              </div>
              <h3 class="text-xl font-semibold mb-3">{feature.title}</h3>
              <p class="text-muted-foreground">{feature.description}</p>
            </Card.Content>
          </Card.Root>
        {/each}
      </div>
    </div>
  </section>

  <!-- Stats Section -->
  <section class="py-20 bg-muted/50">
    <div class="container mx-auto px-4">
      <div class="grid gap-8 md:grid-cols-4 text-center">
        <div>
          <div class="text-4xl font-bold text-primary mb-2">10,000+</div>
          <div class="text-muted-foreground">Active Users</div>
        </div>
        <div>
          <div class="text-4xl font-bold text-primary mb-2">50,000+</div>
          <div class="text-muted-foreground">Projects Created</div>
        </div>
        <div>
          <div class="text-4xl font-bold text-primary mb-2">99.9%</div>
          <div class="text-muted-foreground">Uptime</div>
        </div>
        <div>
          <div class="text-4xl font-bold text-primary mb-2">24/7</div>
          <div class="text-muted-foreground">Support</div>
        </div>
      </div>
    </div>
  </section>

  <!-- Pricing Section -->
  <section class="py-20 lg:py-32">
    <div class="container mx-auto px-4">
      <div class="text-center mb-16">
        <h2 class="text-3xl font-bold tracking-tight sm:text-4xl mb-4">
          Simple, transparent pricing
        </h2>
        <p class="text-xl text-muted-foreground max-w-2xl mx-auto">
          Choose the plan that's right for you. Upgrade or downgrade at any
          time.
        </p>
      </div>

      <div class="grid gap-8 lg:grid-cols-3 max-w-5xl mx-auto">
        {#each pricing as plan (plan.name)}
          <Card.Root
            class="relative {plan.popular ? 'border-primary shadow-lg' : ''}"
          >
            {#if plan.popular}
              <div class="absolute -top-3 left-1/2 -translate-x-1/2">
                <div
                  class="inline-flex items-center rounded-full bg-primary px-3 py-1 text-sm font-medium text-primary-foreground"
                >
                  Most Popular
                </div>
              </div>
            {/if}

            <Card.Content class="p-8">
              <div class="text-center mb-8">
                <h3 class="text-2xl font-bold mb-2">{plan.name}</h3>
                <p class="text-muted-foreground mb-4">{plan.description}</p>
                <div class="flex items-baseline justify-center">
                  <span class="text-4xl font-bold">{plan.price}</span>
                  <span class="text-muted-foreground ml-1">/{plan.period}</span>
                </div>
              </div>

              <ul class="space-y-3 mb-8">
                {#each plan.features as feature (feature)}
                  <li class="flex items-center">
                    <Check class="h-4 w-4 text-primary mr-3 shrink-0" />
                    <span class="text-sm">{feature}</span>
                  </li>
                {/each}
              </ul>

              <Button
                class="w-full"
                variant={plan.popular ? "default" : "outline"}
                size="lg"
                href="/signup"
              >
                {plan.name === "Enterprise" ? "Contact Sales" : "Get Started"}
              </Button>
            </Card.Content>
          </Card.Root>
        {/each}
      </div>
    </div>
  </section>

  <!-- CTA Section -->
  <section class="py-20 lg:py-32 bg-linear-to-r from-blue-600 to-purple-600">
    <div class="container mx-auto px-4 text-center">
      <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl mb-4">
        Ready to get started?
      </h2>
      <p class="text-xl text-blue-100 mb-8 max-w-2xl mx-auto">
        Join thousands of teams already using our platform to manage their
        projects more effectively.
      </p>
      <div class="flex flex-col sm:flex-row gap-4 justify-center">
        <Button
          size="lg"
          variant="secondary"
          href="/signup"
          class="text-lg px-8 py-6"
        >
          Start Free Trial
          <ArrowRight class="ml-2 h-5 w-5" />
        </Button>
        <Button
          size="lg"
          variant="outline"
          href="/login"
          class="text-lg px-8 py-6 border-white text-white hover:bg-white hover:text-blue-600"
        >
          Sign In
        </Button>
      </div>
    </div>
  </section>

  <!-- Footer -->
  <footer class="border-t bg-muted/50">
    <div class="container mx-auto px-4 py-12">
      <div class="grid gap-8 md:grid-cols-4">
        <div class="md:col-span-1">
          <div class="flex items-center space-x-2 mb-4">
            <div
              class="h-8 w-8 rounded bg-primary flex items-center justify-center"
            >
              <span class="text-primary-foreground font-bold">S</span>
            </div>
            <span class="text-xl font-bold">SaaS Boilerplate</span>
          </div>
          <p class="text-muted-foreground text-sm">
            The modern way to manage projects and collaborate with your team.
          </p>
        </div>

        <div>
          <h4 class="font-semibold mb-4">Product</h4>
          <ul class="space-y-2 text-sm text-muted-foreground">
            <li>
              <a href="#" class="hover:text-foreground transition-colors"
                >Features</a
              >
            </li>
            <li>
              <a href="#" class="hover:text-foreground transition-colors"
                >Pricing</a
              >
            </li>
            <li>
              <a href="#" class="hover:text-foreground transition-colors">API</a
              >
            </li>
            <li>
              <a href="#" class="hover:text-foreground transition-colors"
                >Integrations</a
              >
            </li>
          </ul>
        </div>

        <div>
          <h4 class="font-semibold mb-4">Company</h4>
          <ul class="space-y-2 text-sm text-muted-foreground">
            <li>
              <a href="#" class="hover:text-foreground transition-colors"
                >About</a
              >
            </li>
            <li>
              <a href="#" class="hover:text-foreground transition-colors"
                >Blog</a
              >
            </li>
            <li>
              <a href="#" class="hover:text-foreground transition-colors"
                >Careers</a
              >
            </li>
            <li>
              <a href="#" class="hover:text-foreground transition-colors"
                >Contact</a
              >
            </li>
          </ul>
        </div>

        <div>
          <h4 class="font-semibold mb-4">Resources</h4>
          <ul class="space-y-2 text-sm text-muted-foreground">
            <li>
              <a href="#" class="hover:text-foreground transition-colors"
                >Documentation</a
              >
            </li>
            <li>
              <a href="#" class="hover:text-foreground transition-colors"
                >Help Center</a
              >
            </li>
            <li>
              <a href="#" class="hover:text-foreground transition-colors"
                >Privacy</a
              >
            </li>
            <li>
              <a href="#" class="hover:text-foreground transition-colors"
                >Terms</a
              >
            </li>
          </ul>
        </div>
      </div>

      <div
        class="border-t mt-8 pt-8 flex flex-col sm:flex-row justify-between items-center"
      >
        <p class="text-sm text-muted-foreground">
          © 2024 SaaS Boilerplate. All rights reserved.
        </p>
        <div class="flex items-center space-x-4 mt-4 sm:mt-0">
          <a
            href="#"
            class="text-muted-foreground hover:text-foreground transition-colors"
          >
            <Github class="h-5 w-5" />
          </a>
        </div>
      </div>
    </div>
  </footer>
</div>
