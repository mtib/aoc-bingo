import type { ComponentPropsWithoutRef } from "react";
import Disclaimer from "./Disclaimer";

export default function Footer({ className, ...props }: ComponentPropsWithoutRef<'div'>) {
    return (
        <div className={`${className} mt-5`} {...props}>
            <Disclaimer />
        </div>
    )
}