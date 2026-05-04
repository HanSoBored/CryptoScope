import { Button as ButtonPrimitive } from "@base-ui/react/button"
import { cva, type VariantProps } from "class-variance-authority"

import { cn } from "@/lib/utils"

const buttonVariants = cva(
  "group/button inline-flex shrink-0 items-center justify-center rounded-md bg-clip-padding text-sm font-medium whitespace-nowrap transition-all outline-none select-none focus-visible:ring-2 focus-visible:ring-ring/50 active:translate-y-px disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
  {
    variants: {
      variant: {
        default: 
          "bg-primary text-primary-foreground hover:bg-primary/90 focus-visible:ring-primary",
        primary:
          "bg-stitch-primary text-stitch-primary-foreground hover:bg-stitch-primary/90 focus-visible:ring-stitch-primary font-semibold",
        secondary:
          "bg-stitch-secondary text-stitch-secondary-foreground hover:bg-stitch-secondary/90 focus-visible:ring-stitch-secondary font-semibold",
        outline:
          "border border-stitch-border bg-background hover:bg-muted hover:text-foreground focus-visible:ring-ring",
        ghost:
          "hover:bg-muted hover:text-foreground focus-visible:ring-ring",
        destructive:
          "bg-destructive text-destructive-foreground hover:bg-destructive/90 focus-visible:ring-destructive",
        link: "text-primary underline-offset-4 hover:underline",
      },
      size: {
        default:
          "h-9 gap-2 px-4 has-data-[icon=inline-end]:pr-3 has-data-[icon=inline-start]:pl-3",
        xs: "h-7 gap-1.5 rounded-[min(var(--stitch-radius-sm),8px)] px-2.5 text-xs has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 [&_svg:not([class*='size-'])]:size-3",
        sm: "h-8 gap-1.5 rounded-[min(var(--stitch-radius-md),10px)] px-3 text-[0.8rem] has-data-[icon=inline-end]:pr-2.5 has-data-[icon=inline-start]:pl-2.5 [&_svg:not([class*='size-'])]:size-3.5",
        lg: "h-10 gap-2.5 px-5 has-data-[icon=inline-end]:pr-4 has-data-[icon=inline-start]:pl-4 font-semibold",
        icon: "size-9",
        "icon-xs":
          "size-7 rounded-[min(var(--stitch-radius-sm),8px)] [&_svg:not([class*='size-'])]:size-3",
        "icon-sm":
          "size-8 rounded-[min(var(--stitch-radius-md),10px)]",
        "icon-lg": "size-10",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
)

function Button({
  className,
  variant = "default",
  size = "default",
  ...props
}: ButtonPrimitive.Props & VariantProps<typeof buttonVariants>) {
  return (
    <ButtonPrimitive
      data-slot="button"
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  )
}

export { Button, buttonVariants }
