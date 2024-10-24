import {z} from 'zod';
import {createBaseProvider} from '../create-base-provider';
import {onProviderEmit} from '~/desktop';
import type {
    MediaOutput,
    MediaProvider,
    MediaProviderConfig,
} from './media-provider-types';

const mediaProviderConfigSchema = z.object({
    type: z.literal('media'),
    refreshInterval: z.coerce.number().default(5 * 1000),
});

export function createMediaProvider(
    config: MediaProviderConfig,
): MediaProvider {
    const mergedConfig = mediaProviderConfigSchema.parse(config);

    return createBaseProvider(mergedConfig, async queue => {
        return onProviderEmit<MediaOutput>(mergedConfig, ({result}) => {
            if ('error' in result) {
                queue.error(result.error);
            } else {
                queue.output(result.output);
            }
        });
    });
}
