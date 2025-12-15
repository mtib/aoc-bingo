import { type ComponentPropsWithoutRef } from 'react'

export default function Disclaimer({ className, ...props }: ComponentPropsWithoutRef<'div'>) {
    return (
        <div className={className ? `quiet ${className}` : 'quiet'} {...props}>
            This is not associated with <a href="https://adventofcode.com/">Advent of Code</a>. This is a 3rd party project.
        </div>
    )
}