export const formatPath = (path: string): string => {
    let tmp_path = path.replace(/\\/g, '/');
    const segments = tmp_path.split('/');
    
    return segments.slice(0, -1).join('/');
}