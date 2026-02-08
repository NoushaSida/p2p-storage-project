const gulp        = require('gulp');
const fileinclude = require('gulp-file-include');

const paths = {
  scripts: {
    src: 'src/',
    dest: './templates/'
  }
};

async function includeHTML(){
    return gulp.src(//paths.scripts.src
    [
      paths.scripts.src + '*.html',
      !paths.scripts.src + 'includes*'
    ])
    .pipe(fileinclude({
    prefix: '@@',
    basepath: '@file'
    }))
    .pipe(gulp.dest(paths.scripts.dest));
}

exports.default = includeHTML;