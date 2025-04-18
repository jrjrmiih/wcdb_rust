module.exports = {
    extends: ['@commitlint/config-conventional'],
    rules: {
        'header-max-length': [2, 'always', 100],
        'subject-full-stop': [2, 'always', '.'],
    },
    parserPreset: {
        parserOpts: {
            headerPattern: /^(\w*)(?:\((.*)\))?: (.*[.!?])$/,
            headerCorrespondence: ['type', 'scope', 'subject'],
        },
    },
};
