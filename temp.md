# Air Resistance: Quadratic Drag

This is what we usually mean by **Air Resistance**. It accounts for high speeds and the "wind" hitting the surface area of the object.

## Formula

The realistic drag force is calculated as:

$$F_{drag} = -\frac{1}{2} \rho v^2 C_d A \hat{v}$$

Where:
- $\rho$: Fluid density (e.g., air)
- $v$: Velocity of the object
- $C_d$: Drag coefficient (shape-dependent)
- $A$: Cross-sectional area
- $\hat{v}$: Unit vector of velocity (indicates direction)

### Simplified for Code

In simulations, we often simplify this to:

$$F_{drag} = -k \cdot |v| \cdot v$$

Where $k$ encapsulates all constants ($\frac{1}{2} \rho C_d A$).

## Logic
The force increases with the **square of the speed**. This means doubling the speed quadruples the resistance.

## Effect: Terminal Velocity
This creates a **Terminal Velocity**—a point where the force of gravity and air resistance balance out. At this stage, the particle stops accelerating downwards and maintains a constant speed.
